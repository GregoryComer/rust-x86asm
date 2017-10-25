use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpandq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 219, 242], OperandSize::Dword)
}

fn vpandq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 143511636, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 219, 52, 117, 84, 208, 141, 8], OperandSize::Dword)
}

fn vpandq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 157, 219, 1], OperandSize::Dword)
}

fn vpandq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 149, 133, 219, 254], OperandSize::Qword)
}

fn vpandq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1315902800, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 133, 143, 219, 60, 157, 80, 21, 111, 78], OperandSize::Qword)
}

fn vpandq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 1166796532, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 213, 145, 219, 188, 219, 244, 230, 139, 69], OperandSize::Qword)
}

fn vpandq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 175, 219, 235], OperandSize::Dword)
}

fn vpandq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 196705639, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 221, 171, 219, 60, 117, 103, 125, 185, 11], OperandSize::Dword)
}

fn vpandq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 479334239, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 186, 219, 36, 133, 95, 15, 146, 28], OperandSize::Dword)
}

fn vpandq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 213, 162, 219, 213], OperandSize::Qword)
}

fn vpandq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectDisplaced(RCX, 1578381246, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 189, 174, 219, 185, 190, 47, 20, 94], OperandSize::Qword)
}

fn vpandq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 188, 219, 27], OperandSize::Qword)
}

fn vpandq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 229, 202, 219, 242], OperandSize::Dword)
}

fn vpandq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EBX, 1244985849, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 219, 187, 249, 249, 52, 74], OperandSize::Dword)
}

fn vpandq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 221, 219, 19], OperandSize::Dword)
}

fn vpandq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 229, 201, 219, 223], OperandSize::Qword)
}

fn vpandq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM11)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 165, 203, 219, 3], OperandSize::Qword)
}

fn vpandq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1075511472, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 237, 217, 219, 28, 85, 176, 0, 27, 64], OperandSize::Qword)
}

