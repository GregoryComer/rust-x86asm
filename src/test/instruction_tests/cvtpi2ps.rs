use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtpi2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 253], OperandSize::Word)
}

fn cvtpi2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 42, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 81, 42], OperandSize::Word)
}

fn cvtpi2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 220], OperandSize::Dword)
}

fn cvtpi2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 28, 112], OperandSize::Dword)
}

fn cvtpi2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 210], OperandSize::Qword)
}

fn cvtpi2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 4, 81], OperandSize::Qword)
}

