use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 172, 235], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 566142152, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 172, 4, 213, 200, 164, 190, 33], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 172, 213], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RBX, 1236664404, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 172, 131, 84, 0, 182, 73], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 172, 242], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 172, 4, 215], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 172, 224], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 172, 44, 184], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 140, 172, 195], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 722141618, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 172, 36, 253, 178, 1, 11, 43], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 148059411, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 154, 172, 172, 127, 19, 53, 211, 8], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 13, 143, 172, 237], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM14)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 13, 141, 172, 62], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM25)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 53, 147, 172, 24], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 172, 172, 245], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 170, 172, 20, 217], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EAX, 2699387, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 187, 172, 128, 123, 48, 41, 0], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 125, 174, 172, 215], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 1163569510, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 37, 169, 172, 188, 79, 102, 169, 90, 69], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1320086942, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 29, 189, 172, 180, 81, 158, 237, 174, 78], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 187, 172, 247], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 1661391412, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 205, 172, 132, 199, 52, 210, 6, 99], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 219, 172, 31], OperandSize::Dword)
}

#[test]
fn vfnmadd213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 125, 157, 172, 219], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectDisplaced(RDX, 714363042, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 61, 203, 172, 170, 162, 80, 148, 42], OperandSize::Qword)
}

#[test]
fn vfnmadd213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 117, 223, 172, 7], OperandSize::Qword)
}

