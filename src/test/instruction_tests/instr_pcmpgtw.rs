use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpgtw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 205], OperandSize::Dword)
}

#[test]
fn pcmpgtw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 20, 214], OperandSize::Dword)
}

#[test]
fn pcmpgtw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 238], OperandSize::Qword)
}

#[test]
fn pcmpgtw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(MM6)), operand2: Some(IndirectDisplaced(RDX, 158498770, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 101, 178, 210, 127, 114, 9], OperandSize::Qword)
}

#[test]
fn pcmpgtw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 200], OperandSize::Dword)
}

#[test]
fn pcmpgtw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 9], OperandSize::Dword)
}

#[test]
fn pcmpgtw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 194], OperandSize::Qword)
}

#[test]
fn pcmpgtw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 983285128, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 101, 4, 117, 136, 189, 155, 58], OperandSize::Qword)
}

