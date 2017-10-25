use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovsqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 142, 34, 222], OperandSize::Dword)
}

fn vpmovsqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledDisplaced(EDX, Four, 1386936568, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 34, 44, 149, 248, 248, 170, 82], OperandSize::Dword)
}

fn vpmovsqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 126, 138, 34, 195], OperandSize::Qword)
}

fn vpmovsqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 701766244, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 34, 188, 137, 100, 26, 212, 41], OperandSize::Qword)
}

fn vpmovsqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 169, 34, 226], OperandSize::Dword)
}

fn vpmovsqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 683231243, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 34, 164, 211, 11, 72, 185, 40], OperandSize::Dword)
}

fn vpmovsqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM19)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 126, 171, 34, 251], OperandSize::Qword)
}

fn vpmovsqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 34, 49], OperandSize::Qword)
}

fn vpmovsqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 34, 203], OperandSize::Dword)
}

fn vpmovsqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledDisplaced(EDX, Four, 74024744, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 34, 36, 149, 40, 135, 105, 4], OperandSize::Dword)
}

fn vpmovsqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(Direct(XMM19)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 126, 201, 34, 235], OperandSize::Qword)
}

fn vpmovsqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQB, operand1: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 34, 60, 91], OperandSize::Qword)
}

