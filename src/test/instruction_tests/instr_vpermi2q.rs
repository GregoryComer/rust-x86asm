use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2q_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 143, 118, 227], OperandSize::Dword)
}

#[test]
fn vpermi2q_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 118, 42], OperandSize::Dword)
}

#[test]
fn vpermi2q_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 987396956, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 154, 118, 156, 255, 92, 123, 218, 58], OperandSize::Dword)
}

#[test]
fn vpermi2q_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 157, 140, 118, 255], OperandSize::Qword)
}

#[test]
fn vpermi2q_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDX, 1296173864, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 138, 118, 146, 40, 11, 66, 77], OperandSize::Qword)
}

#[test]
fn vpermi2q_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 145, 118, 44, 185], OperandSize::Qword)
}

#[test]
fn vpermi2q_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 171, 118, 221], OperandSize::Dword)
}

#[test]
fn vpermi2q_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 553676992, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 174, 118, 180, 151, 192, 112, 0, 33], OperandSize::Dword)
}

#[test]
fn vpermi2q_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1891761635, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 191, 118, 12, 93, 227, 253, 193, 112], OperandSize::Dword)
}

#[test]
fn vpermi2q_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 141, 173, 118, 228], OperandSize::Qword)
}

#[test]
fn vpermi2q_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 377367577, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 133, 171, 118, 4, 149, 25, 44, 126, 22], OperandSize::Qword)
}

#[test]
fn vpermi2q_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 61240735, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 191, 118, 28, 221, 159, 117, 166, 3], OperandSize::Qword)
}

#[test]
fn vpermi2q_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 204, 118, 195], OperandSize::Dword)
}

#[test]
fn vpermi2q_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 205, 118, 60, 126], OperandSize::Dword)
}

#[test]
fn vpermi2q_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1107205586, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 218, 118, 52, 253, 210, 157, 254, 65], OperandSize::Dword)
}

#[test]
fn vpermi2q_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 173, 204, 118, 240], OperandSize::Qword)
}

#[test]
fn vpermi2q_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 139052599, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 189, 201, 118, 164, 146, 55, 198, 73, 8], OperandSize::Qword)
}

#[test]
fn vpermi2q_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2Q, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectDisplaced(RBX, 291412397, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 205, 214, 118, 163, 173, 153, 94, 17], OperandSize::Qword)
}

