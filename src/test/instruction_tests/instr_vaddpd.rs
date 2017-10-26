use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 88, 202], OperandSize::Dword)
}

#[test]
fn vaddpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 88, 58], OperandSize::Dword)
}

#[test]
fn vaddpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 88, 243], OperandSize::Qword)
}

#[test]
fn vaddpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RAX, 123896429, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 88, 176, 109, 130, 98, 7], OperandSize::Qword)
}

#[test]
fn vaddpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 88, 193], OperandSize::Dword)
}

#[test]
fn vaddpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 88, 36, 217], OperandSize::Dword)
}

#[test]
fn vaddpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 88, 212], OperandSize::Qword)
}

#[test]
fn vaddpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RCX, 1160005887, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 88, 169, 255, 72, 36, 69], OperandSize::Qword)
}

#[test]
fn vaddpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 141, 88, 215], OperandSize::Dword)
}

#[test]
fn vaddpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 1355112413, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 143, 88, 188, 249, 221, 95, 197, 80], OperandSize::Dword)
}

#[test]
fn vaddpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1238015488, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 205, 154, 88, 28, 205, 0, 158, 202, 73], OperandSize::Dword)
}

#[test]
fn vaddpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 157, 142, 88, 236], OperandSize::Qword)
}

#[test]
fn vaddpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RDX, 527235624, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 133, 143, 88, 154, 40, 250, 108, 31], OperandSize::Qword)
}

#[test]
fn vaddpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RBX, 363462957, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 221, 151, 88, 187, 45, 1, 170, 21], OperandSize::Qword)
}

#[test]
fn vaddpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 170, 88, 227], OperandSize::Dword)
}

#[test]
fn vaddpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1842916280, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 171, 88, 52, 253, 184, 171, 216, 109], OperandSize::Dword)
}

#[test]
fn vaddpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 187, 88, 11], OperandSize::Dword)
}

#[test]
fn vaddpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 189, 164, 88, 197], OperandSize::Qword)
}

#[test]
fn vaddpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 1347172207, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 221, 167, 88, 164, 127, 111, 55, 76, 80], OperandSize::Qword)
}

#[test]
fn vaddpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectDisplaced(RBX, 1063696714, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 229, 182, 88, 139, 74, 185, 102, 63], OperandSize::Qword)
}

#[test]
fn vaddpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 222, 88, 228], OperandSize::Dword)
}

#[test]
fn vaddpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 205, 88, 44, 86], OperandSize::Dword)
}

#[test]
fn vaddpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 699442104, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 221, 88, 172, 73, 184, 163, 176, 41], OperandSize::Dword)
}

#[test]
fn vaddpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 253, 151, 88, 231], OperandSize::Qword)
}

#[test]
fn vaddpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 245, 205, 88, 9], OperandSize::Qword)
}

#[test]
fn vaddpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 78451381, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 189, 222, 88, 4, 221, 181, 18, 173, 4], OperandSize::Qword)
}

