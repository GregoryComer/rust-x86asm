use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn mulpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 215], OperandSize::Dword)
}

fn mulpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1728580241, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 4, 189, 145, 10, 8, 103], OperandSize::Dword)
}

fn mulpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 245], OperandSize::Qword)
}

fn mulpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MULPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1406090988, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 89, 44, 117, 236, 62, 207, 83], OperandSize::Qword)
}

