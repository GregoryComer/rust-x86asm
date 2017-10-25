use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fidivr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectDisplaced(DI, 6514, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 189, 114, 25], OperandSize::Word)
}

fn fidivr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectDisplaced(EDI, 2129979012, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 191, 132, 230, 244, 126], OperandSize::Dword)
}

fn fidivr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledDisplaced(RAX, Four, 1431114664, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 60, 133, 168, 19, 77, 85], OperandSize::Qword)
}

fn fidivr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectDisplaced(BP, 19680, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 190, 224, 76], OperandSize::Word)
}

fn fidivr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1785276275, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 188, 152, 115, 39, 105, 106], OperandSize::Dword)
}

fn fidivr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 60, 208], OperandSize::Qword)
}

