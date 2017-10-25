use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn bndcn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND2)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 210], OperandSize::Dword)
}

fn bndcn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND0)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 0], OperandSize::Dword)
}

fn bndcn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND0)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 199], OperandSize::Qword)
}

fn bndcn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCN, operand1: Some(Direct(BND3)), operand2: Some(IndirectDisplaced(RBX, 1044966871, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 27, 155, 215, 237, 72, 62], OperandSize::Qword)
}

