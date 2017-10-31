use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movntps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPS, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 151183533, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 43, 180, 153, 173, 224, 2, 9], OperandSize::Dword)
}

#[test]
fn movntps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPS, operand1: Some(IndirectDisplaced(RDX, 1452729988, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 43, 130, 132, 230, 150, 86], OperandSize::Qword)
}

