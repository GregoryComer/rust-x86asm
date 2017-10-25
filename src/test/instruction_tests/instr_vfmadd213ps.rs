use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 168, 202], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 168, 63], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 168, 234], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 168, 44, 200], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 168, 221], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 168, 2], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 168, 239], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 168, 4, 120], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 140, 168, 201], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 42491252, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 139, 168, 52, 189, 116, 93, 136, 2], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 153, 168, 38], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 69, 131, 168, 231], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RAX, 593766400, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 5, 140, 168, 184, 0, 40, 100, 35], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 29, 155, 168, 20, 74], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 173, 168, 202], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1560177956, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 170, 168, 20, 181, 36, 109, 254, 92], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ESI, 1250717542, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 189, 168, 182, 102, 111, 140, 74], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 45, 170, 168, 249], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM18)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 109, 165, 168, 63], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RDX, 1887656594, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 186, 168, 138, 146, 90, 131, 112], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 250, 168, 236], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 744846911, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 168, 156, 199, 63, 118, 101, 44], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 223, 168, 52, 127], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 77, 147, 168, 201], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 250465887, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 37, 201, 168, 52, 125, 95, 206, 237, 14], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 1646389556, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 109, 217, 168, 180, 210, 52, 233, 33, 98], OperandSize::Qword)
}

