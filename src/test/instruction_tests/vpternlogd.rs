use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpternlogd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 101, 139, 37, 209, 120], OperandSize::Dword)
}

fn vpternlogd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 85, 143, 37, 28, 210, 84], OperandSize::Dword)
}

fn vpternlogd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 321105428, Some(OperandSize::Dword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 77, 159, 37, 164, 147, 20, 174, 35, 19, 124], OperandSize::Dword)
}

fn vpternlogd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM14)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 67, 69, 130, 37, 246, 30], OperandSize::Qword)
}

fn vpternlogd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 642532599, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 227, 77, 134, 37, 44, 125, 247, 68, 76, 38, 15], OperandSize::Qword)
}

fn vpternlogd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectDisplaced(RAX, 1422741669, Some(OperandSize::Dword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 45, 151, 37, 176, 165, 80, 205, 84, 116], OperandSize::Qword)
}

fn vpternlogd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 85, 170, 37, 213, 69], OperandSize::Dword)
}

fn vpternlogd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1121315544, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 77, 173, 37, 60, 85, 216, 234, 213, 66, 9], OperandSize::Dword)
}

fn vpternlogd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1296175332, Some(OperandSize::Dword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 69, 189, 37, 60, 117, 228, 16, 66, 77, 114], OperandSize::Dword)
}

fn vpternlogd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM14)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 67, 69, 170, 37, 214, 65], OperandSize::Qword)
}

fn vpternlogd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM9)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 227, 53, 175, 37, 24, 100], OperandSize::Qword)
}

fn vpternlogd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectDisplaced(RBX, 375945238, Some(OperandSize::Dword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 45, 178, 37, 171, 22, 120, 104, 22, 96], OperandSize::Qword)
}

fn vpternlogd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 85, 201, 37, 203, 10], OperandSize::Dword)
}

fn vpternlogd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 69, 205, 37, 50, 62], OperandSize::Dword)
}

fn vpternlogd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 69, 222, 37, 7, 116], OperandSize::Dword)
}

fn vpternlogd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM24)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 131, 61, 194, 37, 200, 43], OperandSize::Qword)
}

fn vpternlogd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 21, 194, 37, 4, 219, 38], OperandSize::Qword)
}

fn vpternlogd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTERNLOGD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 1833344907, Some(OperandSize::Dword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 99, 125, 209, 37, 164, 185, 139, 159, 70, 109, 41], OperandSize::Qword)
}

