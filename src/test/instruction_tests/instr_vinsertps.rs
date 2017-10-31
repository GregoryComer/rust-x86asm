use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vinsertps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 33, 197, 81], OperandSize::Dword)
}

#[test]
fn vinsertps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 982191978, Some(OperandSize::Dword), None)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 33, 60, 133, 106, 15, 139, 58, 90], OperandSize::Dword)
}

#[test]
fn vinsertps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 33, 192, 59], OperandSize::Qword)
}

#[test]
fn vinsertps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 33, 44, 247, 102], OperandSize::Qword)
}

#[test]
fn vinsertps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 33, 221, 8], OperandSize::Dword)
}

#[test]
fn vinsertps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 33, 11, 66], OperandSize::Dword)
}

#[test]
fn vinsertps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 85, 0, 33, 192, 76], OperandSize::Qword)
}

#[test]
fn vinsertps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 13, 0, 33, 34, 1], OperandSize::Qword)
}

