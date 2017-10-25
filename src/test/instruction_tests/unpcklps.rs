use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn unpcklps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 192], OperandSize::Dword)
}

fn unpcklps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 32], OperandSize::Dword)
}

fn unpcklps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 210], OperandSize::Qword)
}

fn unpcklps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1941356537, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 20, 157, 249, 191, 182, 115], OperandSize::Qword)
}

