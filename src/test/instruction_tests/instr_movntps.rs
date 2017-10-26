use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movntps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPS, operand1: Some(IndirectScaledDisplaced(EBX, Four, 1072739319, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 43, 36, 157, 247, 179, 240, 63], OperandSize::Dword)
}

#[test]
fn movntps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPS, operand1: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 43, 38], OperandSize::Qword)
}

