use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmpsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 194, 222, 23], OperandSize::Dword)
}

#[test]
fn vcmpsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 194, 63, 8], OperandSize::Dword)
}

#[test]
fn vcmpsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 194, 222, 53], OperandSize::Qword)
}

#[test]
fn vcmpsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 314914014, Some(OperandSize::Qword), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 194, 180, 241, 222, 52, 197, 18, 66], OperandSize::Qword)
}

#[test]
fn vcmpsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 207, 31, 194, 203, 83], OperandSize::Dword)
}

#[test]
fn vcmpsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 845837983, Some(OperandSize::Qword), None)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 231, 13, 194, 52, 213, 159, 118, 106, 50, 121], OperandSize::Dword)
}

#[test]
fn vcmpsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 207, 17, 194, 216, 94], OperandSize::Qword)
}

#[test]
fn vcmpsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPSD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 135, 14, 194, 44, 217, 56], OperandSize::Qword)
}

