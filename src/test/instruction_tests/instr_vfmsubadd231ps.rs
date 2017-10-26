use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 183, 245], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1003581505, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 183, 44, 149, 65, 112, 209, 59], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 183, 193], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 183, 50], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 183, 195], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 183, 12, 242], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 183, 209], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 573860102, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 183, 164, 65, 6, 105, 52, 34], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 142, 183, 231], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 143, 183, 62], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 153, 183, 59], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 85, 143, 183, 237], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 101, 131, 183, 47], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM9)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 53, 157, 183, 40], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 183, 229], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1847471863, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 172, 183, 164, 177, 247, 46, 30, 110], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 1713285926, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 191, 183, 148, 65, 38, 171, 30, 102], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 101, 171, 183, 212], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RCX, 1790442688, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 85, 174, 183, 153, 192, 252, 183, 106], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 1312215799, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 45, 185, 183, 188, 216, 247, 210, 54, 78], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 222, 183, 225], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 1362656589, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 202, 183, 140, 88, 77, 125, 56, 81], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EDI, 553083109, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 221, 183, 175, 229, 96, 247, 32], OperandSize::Dword)
}

#[test]
fn vfmsubadd231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 190, 183, 204], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM30)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 13, 199, 183, 9], OperandSize::Qword)
}

#[test]
fn vfmsubadd231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 21, 209, 183, 20, 86], OperandSize::Qword)
}

