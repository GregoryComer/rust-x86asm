use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn unpckhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 242], OperandSize::Dword)
}

fn unpckhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 992522375, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 140, 127, 135, 176, 40, 59], OperandSize::Dword)
}

fn unpckhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 198], OperandSize::Qword)
}

fn unpckhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKHPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 1807301966, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 21, 180, 152, 78, 61, 185, 107], OperandSize::Qword)
}

