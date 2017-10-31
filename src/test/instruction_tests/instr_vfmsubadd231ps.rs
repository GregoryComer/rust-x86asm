use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 183, 245], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1491113398, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 183, 52, 141, 182, 149, 224, 88], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 183, 255], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1091103131, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 183, 52, 213, 155, 233, 8, 65], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 183, 202], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 46100536, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 183, 60, 149, 56, 112, 191, 2], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 183, 217], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 1269251489, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 183, 132, 88, 161, 61, 167, 75], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 137, 183, 239], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1156801991, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 137, 183, 28, 85, 199, 101, 243, 68], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1875339560, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 77, 154, 183, 188, 113, 40, 105, 199, 111], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 85, 135, 183, 239], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 536179610, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 77, 129, 183, 36, 69, 154, 115, 245, 31], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectDisplaced(RCX, 2009999924, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 21, 150, 183, 177, 52, 42, 206, 119], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 169, 183, 208], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 173, 183, 4, 243], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ECX, 139260869, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 185, 183, 177, 197, 243, 76, 8], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 101, 175, 183, 252], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RAX, 783667516, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 117, 173, 183, 168, 60, 209, 181, 46], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 21, 185, 183, 4, 121], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 255, 183, 241], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 206, 183, 36, 191], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1683553194, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 218, 183, 164, 191, 170, 251, 88, 100], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 45, 159, 183, 247], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 93, 203, 183, 12, 223], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM19)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 213, 183, 14], OperandSize::Qword)
}

