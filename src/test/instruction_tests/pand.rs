use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pand_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 198], OperandSize::Dword)
}

fn pand_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 415106318, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 172, 203, 14, 5, 190, 24], OperandSize::Dword)
}

fn pand_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 248], OperandSize::Qword)
}

fn pand_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(MM7)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 219, 56], OperandSize::Qword)
}

fn pand_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 207], OperandSize::Dword)
}

fn pand_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 34], OperandSize::Dword)
}

fn pand_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 216], OperandSize::Qword)
}

fn pand_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PAND, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1306895619, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 219, 52, 189, 3, 165, 229, 77], OperandSize::Qword)
}

