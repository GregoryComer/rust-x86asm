use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 158, 242], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EAX, 1185003322, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 158, 160, 58, 183, 161, 70], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 158, 213], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 158, 44, 127], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 158, 231], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 158, 48], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 158, 212], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 158, 9], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 141, 158, 200], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 1044037460, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 158, 184, 84, 191, 58, 62], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 367006102, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 153, 158, 36, 157, 150, 17, 224, 21], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 133, 132, 158, 245], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 1106953426, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 139, 158, 132, 255, 210, 196, 250, 65], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 237, 157, 158, 20, 127], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 171, 158, 214], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 539844827, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 172, 158, 188, 192, 219, 96, 45, 32], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ESI, 1416805805, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 187, 158, 158, 173, 189, 114, 84], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 237, 171, 158, 244], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM15)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 133, 175, 158, 15], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 73585438, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 149, 187, 158, 164, 74, 30, 211, 98, 4], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 153, 158, 239], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 204, 158, 47], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 218, 158, 60, 184], OperandSize::Dword)
}

#[test]
fn vfnmsub132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 165, 252, 158, 226], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 592385526, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 197, 193, 158, 60, 189, 246, 21, 79, 35], OperandSize::Qword)
}

#[test]
fn vfnmsub132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 524213961, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 237, 217, 158, 148, 203, 201, 222, 62, 31], OperandSize::Qword)
}

