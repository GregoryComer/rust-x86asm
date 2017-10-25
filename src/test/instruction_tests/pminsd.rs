use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pminsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 205], OperandSize::Dword)
}

fn pminsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1417000176, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 44, 149, 240, 180, 117, 84], OperandSize::Dword)
}

fn pminsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 238], OperandSize::Qword)
}

fn pminsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 57, 30], OperandSize::Qword)
}

