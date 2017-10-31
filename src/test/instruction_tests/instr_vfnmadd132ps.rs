use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 156, 204], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 156, 54], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 156, 244], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDI, 1157228305, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 156, 143, 17, 231, 249, 68], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 156, 216], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 755453, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 156, 164, 129, 253, 134, 11, 0], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 156, 233], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 1073302407, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 156, 156, 219, 135, 75, 249, 63], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 141, 156, 222], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 66526273, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 143, 156, 4, 221, 65, 28, 247, 3], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 158, 156, 52, 191], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 37, 137, 156, 251], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 85, 140, 156, 44, 184], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectDisplaced(RAX, 617486623, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 21, 148, 156, 184, 31, 25, 206, 36], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 171, 156, 235], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ECX, 1143190740, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 169, 156, 137, 212, 180, 35, 68], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 450183844, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 187, 156, 140, 251, 164, 66, 213, 26], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 69, 173, 156, 233], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1248378074, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 29, 163, 156, 180, 66, 218, 188, 104, 74], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 69, 190, 156, 44, 79], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 253, 156, 239], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 1516443145, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 204, 156, 172, 82, 9, 22, 99, 90], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1831511985, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 219, 156, 172, 215, 177, 167, 42, 109], OperandSize::Dword)
}

#[test]
fn vfnmadd132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 37, 212, 156, 219], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 311654746, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 69, 193, 156, 36, 69, 90, 121, 147, 18], OperandSize::Qword)
}

#[test]
fn vfnmadd132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD132PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectDisplaced(RDI, 2109550, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 21, 214, 156, 191, 110, 48, 32, 0], OperandSize::Qword)
}

