use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcomisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 221], OperandSize::Dword)
}

#[test]
fn vcomisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 41], OperandSize::Dword)
}

#[test]
fn vcomisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 197], OperandSize::Qword)
}

#[test]
fn vcomisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 20, 241], OperandSize::Qword)
}

#[test]
fn vcomisd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 253, 24, 47, 211], OperandSize::Dword)
}

#[test]
fn vcomisd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 559994804, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 12, 133, 180, 215, 96, 33], OperandSize::Dword)
}

#[test]
fn vcomisd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 65, 253, 24, 47, 229], OperandSize::Qword)
}

#[test]
fn vcomisd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 2056009310, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 148, 251, 94, 54, 140, 122], OperandSize::Qword)
}

