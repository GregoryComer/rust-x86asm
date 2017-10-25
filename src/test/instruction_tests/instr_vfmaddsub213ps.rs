use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 166, 244], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 519087060, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 166, 144, 212, 163, 240, 30], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 166, 244], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 127322505, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 166, 156, 67, 137, 201, 150, 7], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 166, 209], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 166, 41], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 166, 247], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 550770486, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 166, 60, 245, 54, 23, 212, 32], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 138, 166, 248], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 141, 166, 43], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 645442043, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 159, 166, 180, 153, 251, 169, 120, 38], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 37, 139, 166, 197], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectDisplaced(RDX, 1841736393, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 13, 143, 166, 178, 201, 170, 198, 109], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 45, 158, 166, 47], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 174, 166, 215], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EAX, 1502876359, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 174, 166, 144, 199, 18, 148, 89], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 366067318, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 69, 187, 166, 148, 71, 118, 190, 209, 21], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 21, 174, 166, 194], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 21, 162, 166, 28, 207], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 1908172924, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 125, 183, 166, 148, 120, 124, 104, 188, 113], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 155, 166, 248], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 166, 28, 112], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EBX, 510119726, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 219, 166, 155, 46, 207, 103, 30], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 61, 158, 166, 226], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 338349824, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 202, 166, 28, 253, 0, 207, 42, 20], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 210, 166, 4, 246], OperandSize::Qword)
}

