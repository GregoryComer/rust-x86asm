use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 151, 205], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 151, 1], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 151, 246], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 151, 60, 152], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 151, 194], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 151, 20, 200], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 151, 220], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RDX, 339947607, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 151, 170, 87, 48, 67, 20], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 140, 151, 224], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 143, 151, 60, 210], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EBX, 818296870, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 159, 151, 179, 38, 56, 198, 48], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 181, 137, 151, 247], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RDI, 1402013864, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 237, 138, 151, 159, 168, 8, 145, 83], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 212548203, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 237, 159, 151, 156, 121, 107, 58, 171, 12], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 173, 151, 213], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 173, 151, 12, 183], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 1282770898, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 191, 151, 185, 210, 135, 117, 76], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 133, 173, 151, 253], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1483623378, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 221, 163, 151, 28, 181, 210, 75, 110, 88], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 245, 191, 151, 35], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 185, 151, 250], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 1840197711, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 151, 164, 176, 79, 48, 175, 109], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(ECX, 1809365339, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 220, 151, 177, 91, 185, 216, 107], OperandSize::Dword)
}

#[test]
fn vfmsubadd132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 82, 165, 159, 151, 237], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 640510860, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 194, 151, 140, 201, 140, 107, 45, 38], OperandSize::Qword)
}

#[test]
fn vfmsubadd132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD132PD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 222449422, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 237, 217, 151, 188, 203, 14, 79, 66, 13], OperandSize::Qword)
}

