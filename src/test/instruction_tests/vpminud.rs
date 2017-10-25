use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpminud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 59, 239], OperandSize::Dword)
}

fn vpminud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 916818087, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 59, 130, 167, 136, 165, 54], OperandSize::Dword)
}

fn vpminud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 59, 246], OperandSize::Qword)
}

fn vpminud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1375111078, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 59, 156, 74, 166, 135, 246, 81], OperandSize::Qword)
}

fn vpminud_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 59, 209], OperandSize::Dword)
}

fn vpminud_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 59, 34], OperandSize::Dword)
}

fn vpminud_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 59, 227], OperandSize::Qword)
}

fn vpminud_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RDI, 1667077117, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 59, 159, 253, 147, 93, 99], OperandSize::Qword)
}

fn vpminud_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 139, 59, 233], OperandSize::Dword)
}

fn vpminud_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1853949300, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 142, 59, 20, 197, 116, 5, 129, 110], OperandSize::Dword)
}

fn vpminud_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 803966271, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 156, 59, 28, 221, 63, 141, 235, 47], OperandSize::Dword)
}

fn vpminud_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 45, 143, 59, 217], OperandSize::Qword)
}

fn vpminud_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1106417593, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 69, 139, 59, 4, 133, 185, 151, 242, 65], OperandSize::Qword)
}

fn vpminud_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 880522270, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 93, 150, 59, 12, 77, 30, 180, 123, 52], OperandSize::Qword)
}

fn vpminud_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 59, 217], OperandSize::Dword)
}

fn vpminud_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ESI, 478297600, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 173, 59, 190, 0, 62, 130, 28], OperandSize::Dword)
}

fn vpminud_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 728382078, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 185, 59, 172, 139, 126, 58, 106, 43], OperandSize::Dword)
}

fn vpminud_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 85, 165, 59, 239], OperandSize::Qword)
}

fn vpminud_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RSI, 447123791, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 109, 170, 59, 174, 79, 145, 166, 26], OperandSize::Qword)
}

fn vpminud_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 85, 191, 59, 44, 217], OperandSize::Qword)
}

fn vpminud_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 206, 59, 209], OperandSize::Dword)
}

fn vpminud_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1639466027, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 202, 59, 28, 245, 43, 68, 184, 97], OperandSize::Dword)
}

fn vpminud_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 1562612736, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 221, 59, 164, 207, 0, 148, 35, 93], OperandSize::Dword)
}

fn vpminud_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 109, 204, 59, 237], OperandSize::Qword)
}

fn vpminud_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM22)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 77, 198, 59, 0], OperandSize::Qword)
}

fn vpminud_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectDisplaced(RDX, 234899817, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 125, 215, 59, 162, 105, 73, 0, 14], OperandSize::Qword)
}

