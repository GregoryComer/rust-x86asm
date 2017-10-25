use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pblendw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 249, 99], OperandSize::Dword)
}

fn pblendw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 709272446, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 12, 133, 126, 163, 70, 42, 117], OperandSize::Dword)
}

fn pblendw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 200, 45], OperandSize::Qword)
}

fn pblendw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PBLENDW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RCX, 1751528011, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 14, 185, 75, 50, 102, 104, 30], OperandSize::Qword)
}

