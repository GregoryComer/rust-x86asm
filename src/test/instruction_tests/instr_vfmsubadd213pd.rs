use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 167, 206], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EBX, 921550794, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 167, 163, 202, 191, 237, 54], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 167, 243], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 998710672, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 167, 28, 149, 144, 29, 135, 59], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 167, 250], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 1675885491, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 167, 140, 82, 179, 251, 227, 99], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 167, 235], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RBX, 1654660337, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 167, 155, 241, 28, 160, 98], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 137, 167, 242], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 138, 167, 47], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 159, 167, 22], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 189, 135, 167, 254], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 615229829, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 189, 141, 167, 140, 134, 133, 169, 171, 36], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 829942915, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 165, 151, 167, 36, 69, 131, 236, 119, 49], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 169, 167, 206], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 174, 167, 44, 79], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 188, 167, 20, 178], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 205, 169, 167, 192], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectDisplaced(RSI, 2143035356, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 221, 165, 167, 174, 220, 31, 188, 127], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1216821482, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 149, 178, 167, 52, 69, 234, 56, 135, 72], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 157, 167, 227], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1649406622, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 204, 167, 36, 189, 158, 242, 79, 98], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 221, 167, 14], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 221, 188, 167, 218], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 1772437103, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 189, 205, 167, 140, 207, 111, 62, 165, 105], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 245, 220, 167, 48], OperandSize::Qword)
}

