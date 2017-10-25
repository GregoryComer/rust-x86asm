use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 150, 222], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 139198855, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 150, 156, 199, 135, 1, 76, 8], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 150, 220], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 150, 32], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 150, 255], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 150, 12, 177], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 150, 199], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 619965464, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 150, 140, 65, 24, 236, 243, 36], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 138, 150, 228], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 1873813246, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 139, 150, 130, 254, 30, 176, 111], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 155, 150, 54], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 13, 139, 150, 238], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 1591322601, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 61, 134, 150, 188, 115, 233, 167, 217, 94], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1040740807, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 45, 149, 150, 132, 86, 199, 113, 8, 62], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 150, 229], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1065917843, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 169, 150, 20, 197, 147, 157, 136, 63], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 188, 150, 44, 159], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 29, 163, 150, 210], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1361733494, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 166, 150, 12, 77, 118, 103, 42, 81], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 687333430, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 125, 190, 150, 164, 145, 54, 224, 247, 40], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 189, 150, 216], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1296735884, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 204, 150, 20, 117, 140, 158, 74, 77], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(ECX, 337052009, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 220, 150, 153, 105, 1, 23, 20], OperandSize::Dword)
}

#[test]
fn vfmaddsub132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 85, 253, 150, 233], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 85, 194, 150, 36, 241], OperandSize::Qword)
}

#[test]
fn vfmaddsub132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(RDX, 57075170, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 69, 221, 150, 186, 226, 229, 102, 3], OperandSize::Qword)
}

