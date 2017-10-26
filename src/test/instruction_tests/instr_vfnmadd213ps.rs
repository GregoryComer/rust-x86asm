use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 172, 227], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 172, 36, 215], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 172, 208], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 172, 0], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 172, 220], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 172, 38], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 172, 205], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 172, 4, 182], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 140, 172, 207], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 867464871, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 141, 172, 28, 189, 167, 118, 180, 51], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 2016520049, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 155, 172, 156, 179, 113, 167, 49, 120], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 93, 134, 172, 237], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 117, 129, 172, 12, 142], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 203711599, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 45, 159, 172, 172, 66, 111, 100, 36, 12], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 175, 172, 199], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 172, 172, 36, 191], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1016435196, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 188, 172, 60, 133, 252, 145, 149, 60], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 77, 173, 172, 221], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RDI, 882893310, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 85, 171, 172, 135, 254, 225, 159, 52], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectDisplaced(RBX, 1882480869, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 77, 177, 172, 163, 229, 96, 52, 112], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 249, 172, 220], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1773628993, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 204, 172, 12, 221, 65, 110, 183, 105], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 217, 172, 44, 187], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 125, 214, 172, 247], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 5, 201, 172, 36, 82], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectDisplaced(RAX, 1821175124, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 209, 172, 168, 84, 237, 140, 108], OperandSize::Qword)
}

