use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pavgw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 194], OperandSize::Dword)
}

#[test]
fn pavgw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1313191364, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 52, 205, 196, 181, 69, 78], OperandSize::Dword)
}

#[test]
fn pavgw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 254], OperandSize::Qword)
}

#[test]
fn pavgw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 304956682, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 227, 60, 141, 10, 69, 45, 18], OperandSize::Qword)
}

#[test]
fn pavgw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 252], OperandSize::Dword)
}

#[test]
fn pavgw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 1858282942, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 164, 186, 190, 37, 195, 110], OperandSize::Dword)
}

#[test]
fn pavgw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 253], OperandSize::Qword)
}

#[test]
fn pavgw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PAVGW, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 227, 38], OperandSize::Qword)
}

