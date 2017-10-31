use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ECX, 1886768978, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 22, 177, 82, 207, 117, 112], OperandSize::Dword)
}

#[test]
fn vmovhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 22, 11], OperandSize::Qword)
}

#[test]
fn vmovhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 22, 20, 190], OperandSize::Dword)
}

#[test]
fn vmovhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 593264798, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 84, 0, 22, 4, 125, 158, 128, 92, 35], OperandSize::Qword)
}

#[test]
fn vmovhps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectDisplaced(ECX, 905374652, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 161, 188, 235, 246, 53], OperandSize::Dword)
}

#[test]
fn vmovhps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectDisplaced(RSI, 1953862922, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 166, 10, 149, 117, 116], OperandSize::Qword)
}

#[test]
fn vmovhps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 23, 46], OperandSize::Dword)
}

#[test]
fn vmovhps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPS, operand1: Some(IndirectDisplaced(RCX, 1980248590, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 124, 8, 23, 185, 14, 50, 8, 118], OperandSize::Qword)
}

