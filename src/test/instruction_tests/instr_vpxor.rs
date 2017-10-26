use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpxor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 239, 215], OperandSize::Dword)
}

#[test]
fn vpxor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 91272009, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 239, 52, 77, 73, 179, 112, 5], OperandSize::Dword)
}

#[test]
fn vpxor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 239, 208], OperandSize::Qword)
}

#[test]
fn vpxor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RCX, 70604587, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 239, 185, 43, 87, 53, 4], OperandSize::Qword)
}

#[test]
fn vpxor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 239, 229], OperandSize::Dword)
}

#[test]
fn vpxor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 239, 46], OperandSize::Dword)
}

#[test]
fn vpxor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 239, 226], OperandSize::Qword)
}

#[test]
fn vpxor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXOR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RDX, 1887754694, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 239, 186, 198, 217, 132, 112], OperandSize::Qword)
}

