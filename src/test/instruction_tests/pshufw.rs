use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pshufw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM4)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 220, 73], OperandSize::Word)
}

fn pshufw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(BP, 10778, Some(OperandSize::Qword), None)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 158, 26, 42, 59], OperandSize::Word)
}

fn pshufw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM5)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 221, 37], OperandSize::Dword)
}

fn pshufw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 15, 81], OperandSize::Dword)
}

fn pshufw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 230, 13], OperandSize::Qword)
}

fn pshufw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFW, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(RSI, 537882979, Some(OperandSize::Qword), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 112, 134, 99, 113, 15, 32, 121], OperandSize::Qword)
}

