use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpgtd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 198], OperandSize::Dword)
}

#[test]
fn pcmpgtd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 1595416505, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 180, 126, 185, 31, 24, 95], OperandSize::Dword)
}

#[test]
fn pcmpgtd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 195], OperandSize::Qword)
}

#[test]
fn pcmpgtd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1000678886, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 12, 133, 230, 37, 165, 59], OperandSize::Qword)
}

#[test]
fn pcmpgtd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 243], OperandSize::Dword)
}

#[test]
fn pcmpgtd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 1], OperandSize::Dword)
}

#[test]
fn pcmpgtd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 237], OperandSize::Qword)
}

#[test]
fn pcmpgtd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 12, 247], OperandSize::Qword)
}

