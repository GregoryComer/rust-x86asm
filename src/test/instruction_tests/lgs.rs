use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn lgs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 181, 8], OperandSize::Word)
}

fn lgs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 1568299075, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 181, 180, 151, 67, 88, 122, 93], OperandSize::Dword)
}

fn lgs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(SP)), operand2: Some(Indirect(RSI, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 181, 38], OperandSize::Qword)
}

fn lgs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(ECX, 484018484, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 181, 153, 52, 137, 217, 28], OperandSize::Dword)
}

fn lgs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 279647357, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 181, 180, 128, 125, 20, 171, 16], OperandSize::Qword)
}

fn lgs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Far64), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 181, 44, 150], OperandSize::Qword)
}

