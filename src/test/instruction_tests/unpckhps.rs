use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn unpckhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 232], OperandSize::Dword)
}

fn unpckhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1924074231, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 28, 189, 247, 10, 175, 114], OperandSize::Dword)
}

fn unpckhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 233], OperandSize::Qword)
}

fn unpckhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPS, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 21, 49], OperandSize::Qword)
}

