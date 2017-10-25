use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovsqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 37, 213], OperandSize::Dword)
}

fn vpmovsqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 37, 60, 192], OperandSize::Dword)
}

fn vpmovsqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 126, 137, 37, 193], OperandSize::Qword)
}

fn vpmovsqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 37, 36, 248], OperandSize::Qword)
}

fn vpmovsqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 171, 37, 192], OperandSize::Dword)
}

fn vpmovsqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 617321993, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 37, 52, 213, 9, 150, 203, 36], OperandSize::Dword)
}

fn vpmovsqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 126, 172, 37, 214], OperandSize::Qword)
}

fn vpmovsqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 37, 4, 87], OperandSize::Qword)
}

fn vpmovsqd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 205, 37, 252], OperandSize::Dword)
}

fn vpmovsqd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 335183874, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 37, 36, 205, 2, 128, 250, 19], OperandSize::Dword)
}

fn vpmovsqd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 126, 206, 37, 253], OperandSize::Qword)
}

fn vpmovsqd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 37, 28, 211], OperandSize::Qword)
}

