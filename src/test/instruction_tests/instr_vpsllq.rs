use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 115, 246, 9], OperandSize::Dword)
}

#[test]
fn vpsllq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 115, 241, 48], OperandSize::Qword)
}

#[test]
fn vpsllq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 115, 242, 43], OperandSize::Dword)
}

#[test]
fn vpsllq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 115, 247, 122], OperandSize::Qword)
}

#[test]
fn vpsllq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 243, 235], OperandSize::Dword)
}

#[test]
fn vpsllq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 1952285758, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 243, 145, 62, 132, 93, 116], OperandSize::Dword)
}

#[test]
fn vpsllq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 243, 204], OperandSize::Qword)
}

#[test]
fn vpsllq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 57214568, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 243, 180, 194, 104, 6, 105, 3], OperandSize::Qword)
}

#[test]
fn vpsllq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 243, 230], OperandSize::Dword)
}

#[test]
fn vpsllq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 243, 9], OperandSize::Dword)
}

#[test]
fn vpsllq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 243, 209], OperandSize::Qword)
}

#[test]
fn vpsllq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 243, 60, 183], OperandSize::Qword)
}

#[test]
fn vpsllq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 142, 243, 225], OperandSize::Dword)
}

#[test]
fn vpsllq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 142, 243, 52, 91], OperandSize::Dword)
}

#[test]
fn vpsllq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 237, 134, 243, 236], OperandSize::Qword)
}

#[test]
fn vpsllq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM8)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 189, 142, 243, 27], OperandSize::Qword)
}

#[test]
fn vpsllq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 172, 243, 231], OperandSize::Dword)
}

#[test]
fn vpsllq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 243, 4, 203], OperandSize::Dword)
}

#[test]
fn vpsllq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 229, 174, 243, 228], OperandSize::Qword)
}

#[test]
fn vpsllq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 901699417, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 221, 163, 243, 132, 136, 89, 215, 190, 53], OperandSize::Qword)
}

#[test]
fn vpsllq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 243, 232], OperandSize::Dword)
}

#[test]
fn vpsllq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 1750043482, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 205, 243, 148, 115, 90, 139, 79, 104], OperandSize::Dword)
}

#[test]
fn vpsllq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 229, 202, 243, 254], OperandSize::Qword)
}

#[test]
fn vpsllq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 245, 201, 243, 44, 91], OperandSize::Qword)
}

