use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 105, 246], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 513972147, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 105, 172, 243, 179, 151, 162, 30], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 105, 197], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RDI, 1829586274, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 105, 183, 98, 69, 13, 109], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 105, 249], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 682402985, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 105, 132, 113, 169, 164, 172, 40], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 105, 213], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 565054468, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 105, 28, 213, 4, 12, 174, 33], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 138, 105, 227], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 440223887, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 138, 105, 44, 93, 143, 72, 61, 26], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 69, 134, 105, 192], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 130, 105, 34], OperandSize::Qword)
}

