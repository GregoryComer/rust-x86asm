use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpeqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 206], OperandSize::Dword)
}

#[test]
fn pcmpeqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EDX, 1998165313, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 186, 65, 149, 25, 119], OperandSize::Dword)
}

#[test]
fn pcmpeqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 212], OperandSize::Qword)
}

#[test]
fn pcmpeqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 1508456186, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 117, 164, 119, 250, 54, 233, 89], OperandSize::Qword)
}

#[test]
fn pcmpeqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 233], OperandSize::Dword)
}

#[test]
fn pcmpeqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1322165689, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 164, 134, 185, 165, 206, 78], OperandSize::Dword)
}

#[test]
fn pcmpeqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 221], OperandSize::Qword)
}

#[test]
fn pcmpeqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 358350013, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 117, 52, 85, 189, 252, 91, 21], OperandSize::Qword)
}

