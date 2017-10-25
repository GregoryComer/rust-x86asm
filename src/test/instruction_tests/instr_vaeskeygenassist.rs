use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaeskeygenassist_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 237, 70], OperandSize::Dword)
}

#[test]
fn vaeskeygenassist_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 384798565, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 20, 69, 101, 143, 239, 22, 89], OperandSize::Dword)
}

#[test]
fn vaeskeygenassist_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 253, 120], OperandSize::Qword)
}

#[test]
fn vaeskeygenassist_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESKEYGENASSIST, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 223, 31, 110], OperandSize::Qword)
}

