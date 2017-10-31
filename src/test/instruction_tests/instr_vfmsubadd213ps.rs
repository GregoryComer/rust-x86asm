use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 167, 245], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 2138699849, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 167, 36, 141, 73, 248, 121, 127], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 167, 246], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 167, 34], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 167, 200], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 1250965578, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 167, 180, 139, 74, 56, 144, 74], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 167, 212], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 167, 25], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 137, 167, 245], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDI, 1935469381, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 137, 167, 143, 69, 235, 92, 115], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ESI, 1366847221, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 77, 156, 167, 174, 245, 110, 120, 81], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 117, 131, 167, 239], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RDX, 930367438, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 139, 167, 146, 206, 71, 116, 55], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1581566786, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 37, 149, 167, 4, 205, 66, 203, 68, 94], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 174, 167, 208], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 172, 167, 17], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1260262369, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 190, 167, 20, 93, 225, 19, 30, 75], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 21, 166, 167, 234], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectDisplaced(RCX, 159677962, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 117, 162, 167, 129, 10, 126, 132, 9], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RDX, 377227726, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 85, 188, 167, 162, 206, 9, 124, 22], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 153, 167, 225], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 205, 167, 6], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 1281546851, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 219, 167, 132, 130, 99, 218, 98, 76], OperandSize::Dword)
}

#[test]
fn vfmsubadd213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 93, 153, 167, 195], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 21, 199, 167, 20, 219], OperandSize::Qword)
}

#[test]
fn vfmsubadd213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectDisplaced(RSI, 1022118514, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 117, 209, 167, 158, 114, 74, 236, 60], OperandSize::Qword)
}

