use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 219, 104], OperandSize::Dword)
}

#[test]
fn vextractps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(IndirectDisplaced(EAX, 644925669, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 136, 229, 200, 112, 38, 24], OperandSize::Dword)
}

#[test]
fn vextractps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 209, 31], OperandSize::Qword)
}

#[test]
fn vextractps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 46, 53], OperandSize::Qword)
}

#[test]
fn vextractps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 253, 124], OperandSize::Dword)
}

#[test]
fn vextractps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1320454466, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 44, 133, 66, 137, 180, 78, 45], OperandSize::Dword)
}

#[test]
fn vextractps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM15)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 99, 121, 23, 254, 115], OperandSize::Qword)
}

#[test]
fn vextractps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTPS, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 448189884, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 23, 132, 79, 188, 213, 182, 26, 107], OperandSize::Qword)
}

