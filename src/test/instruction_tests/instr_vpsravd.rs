use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsravd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 70, 205], OperandSize::Dword)
}

#[test]
fn vpsravd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 111661557, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 70, 4, 157, 245, 209, 167, 6], OperandSize::Dword)
}

#[test]
fn vpsravd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 70, 235], OperandSize::Qword)
}

#[test]
fn vpsravd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 438222931, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 70, 52, 181, 83, 192, 30, 26], OperandSize::Qword)
}

#[test]
fn vpsravd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 70, 230], OperandSize::Dword)
}

#[test]
fn vpsravd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ECX, 1861933576, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 70, 153, 8, 218, 250, 110], OperandSize::Dword)
}

#[test]
fn vpsravd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 70, 201], OperandSize::Qword)
}

#[test]
fn vpsravd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RDI, 1546322719, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 70, 175, 31, 3, 43, 92], OperandSize::Qword)
}

#[test]
fn vpsravd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 141, 70, 192], OperandSize::Dword)
}

#[test]
fn vpsravd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 662326612, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 70, 20, 189, 84, 77, 122, 39], OperandSize::Dword)
}

#[test]
fn vpsravd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1986614957, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 158, 70, 148, 242, 173, 86, 105, 118], OperandSize::Dword)
}

#[test]
fn vpsravd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 37, 135, 70, 239], OperandSize::Qword)
}

#[test]
fn vpsravd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 138, 70, 36, 153], OperandSize::Qword)
}

#[test]
fn vpsravd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 53, 149, 70, 60, 90], OperandSize::Qword)
}

#[test]
fn vpsravd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 169, 70, 240], OperandSize::Dword)
}

#[test]
fn vpsravd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EBX, 1513421623, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 171, 70, 187, 55, 251, 52, 90], OperandSize::Dword)
}

#[test]
fn vpsravd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EDI, 801971253, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 188, 70, 183, 53, 28, 205, 47], OperandSize::Dword)
}

#[test]
fn vpsravd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 61, 172, 70, 200], OperandSize::Qword)
}

#[test]
fn vpsravd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectDisplaced(RAX, 137661938, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 101, 161, 70, 168, 242, 141, 52, 8], OperandSize::Qword)
}

#[test]
fn vpsravd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 1938438180, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 93, 190, 70, 132, 127, 36, 56, 138, 115], OperandSize::Qword)
}

#[test]
fn vpsravd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 70, 225], OperandSize::Dword)
}

#[test]
fn vpsravd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1089027939, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 203, 70, 164, 146, 99, 63, 233, 64], OperandSize::Dword)
}

#[test]
fn vpsravd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(ECX, 1990486857, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 221, 70, 177, 73, 107, 164, 118], OperandSize::Dword)
}

#[test]
fn vpsravd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 2, 53, 196, 70, 234], OperandSize::Qword)
}

#[test]
fn vpsravd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectDisplaced(RDX, 214734015, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 21, 207, 70, 170, 191, 148, 204, 12], OperandSize::Qword)
}

#[test]
fn vpsravd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM25)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 53, 214, 70, 42], OperandSize::Qword)
}

