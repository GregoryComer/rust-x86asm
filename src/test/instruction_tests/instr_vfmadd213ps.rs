use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 168, 205], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 168, 20, 219], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 168, 227], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 168, 52, 218], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 168, 218], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 168, 11], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 168, 242], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RBX, 231699799, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 168, 139, 87, 117, 207, 13], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 138, 168, 222], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 141, 168, 4, 210], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 963921680, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 155, 168, 156, 179, 16, 71, 116, 57], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 109, 137, 168, 201], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 873462266, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 69, 142, 168, 28, 253, 250, 249, 15, 52], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1103703841, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 93, 158, 168, 44, 77, 33, 47, 201, 65], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 168, 234], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 174, 168, 58], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EAX, 291280332, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 186, 168, 168, 204, 149, 92, 17], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 166, 168, 209], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 1886436197, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 37, 175, 168, 148, 222, 101, 187, 112, 112], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectDisplaced(RDI, 825611461, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 21, 182, 168, 151, 197, 212, 53, 49], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 157, 168, 217], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1238705004, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 206, 168, 20, 205, 108, 35, 213, 73], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EBX, 640298460, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 220, 168, 187, 220, 45, 42, 38], OperandSize::Dword)
}

#[test]
fn vfmadd213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 125, 185, 168, 242], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 61, 194, 168, 44, 182], OperandSize::Qword)
}

#[test]
fn vfmadd213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD213PS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 482960962, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 53, 223, 168, 52, 133, 66, 102, 201, 28], OperandSize::Qword)
}

