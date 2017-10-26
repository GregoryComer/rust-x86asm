use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 250, 199], OperandSize::Dword)
}

#[test]
fn vpsubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ECX, 2051108328, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 250, 153, 232, 109, 65, 122], OperandSize::Dword)
}

#[test]
fn vpsubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 250, 195], OperandSize::Qword)
}

#[test]
fn vpsubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDX, 2046800276, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 250, 130, 148, 177, 255, 121], OperandSize::Qword)
}

#[test]
fn vpsubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 250, 214], OperandSize::Dword)
}

#[test]
fn vpsubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 250, 43], OperandSize::Dword)
}

#[test]
fn vpsubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 250, 206], OperandSize::Qword)
}

#[test]
fn vpsubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 250, 36, 222], OperandSize::Qword)
}

