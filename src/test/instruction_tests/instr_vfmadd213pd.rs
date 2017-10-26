use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 168, 216], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 55933313, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 168, 164, 118, 129, 121, 85, 3], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 168, 209], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 168, 52, 86], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 168, 204], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 168, 36, 200], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 168, 253], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 504752581, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 168, 140, 255, 197, 233, 21, 30], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 168, 249], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 223456908, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 140, 168, 28, 141, 140, 174, 81, 13], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 157, 168, 60, 146], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 165, 131, 168, 212], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectDisplaced(RSI, 114472866, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 253, 129, 168, 174, 162, 183, 210, 6], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1286171276, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 141, 151, 168, 180, 195, 140, 106, 169, 76], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 175, 168, 192], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 1136028792, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 174, 168, 146, 120, 108, 182, 67], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 188, 168, 36, 200], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 237, 171, 168, 222], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 197, 164, 168, 4, 67], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1388195516, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 245, 179, 168, 36, 93, 188, 46, 190, 82], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 156, 168, 231], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 207, 168, 60, 190], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EAX, 1608046039, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 222, 168, 152, 215, 213, 216, 95], OperandSize::Dword)
}

#[test]
fn vfmadd213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 173, 187, 168, 236], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 1751507298, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 221, 205, 168, 132, 218, 98, 225, 101, 104], OperandSize::Qword)
}

#[test]
fn vfmadd213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 1714547863, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 245, 210, 168, 180, 254, 151, 236, 49, 102], OperandSize::Qword)
}

