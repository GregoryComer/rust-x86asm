use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 168, 244], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1482772366, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 168, 12, 221, 142, 79, 97, 88], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 168, 254], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 168, 12, 70], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 168, 207], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 168, 12, 152], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 168, 220], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 1611819961, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 168, 156, 241, 185, 107, 18, 96], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 138, 168, 254], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1904352730, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 168, 52, 253, 218, 29, 130, 113], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 159, 168, 22], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 157, 138, 168, 194], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectDisplaced(RDX, 1023910783, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 141, 139, 168, 130, 127, 163, 7, 61], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 2131744795, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 237, 157, 168, 44, 77, 27, 216, 15, 127], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 169, 168, 202], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 2065620162, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 175, 168, 188, 75, 194, 220, 30, 123], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 188, 168, 52, 159], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 221, 164, 168, 214], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 355968731, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 157, 175, 168, 60, 181, 219, 166, 55, 21], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 188, 168, 20, 74], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 155, 168, 251], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1214758912, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 207, 168, 60, 181, 0, 192, 103, 72], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EBX, 1602854320, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 219, 168, 147, 176, 157, 137, 95], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 141, 247, 168, 211], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM9)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 181, 204, 168, 27], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectDisplaced(RDX, 1964718636, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 189, 219, 168, 130, 44, 58, 27, 117], OperandSize::Qword)
}

