use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cvtsd2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 237], OperandSize::Dword)
}

fn cvtsd2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EAX, 1675196205, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 136, 45, 119, 217, 99], OperandSize::Dword)
}

fn cvtsd2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 225], OperandSize::Qword)
}

fn cvtsd2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 516227022, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 90, 4, 141, 206, 255, 196, 30], OperandSize::Qword)
}

