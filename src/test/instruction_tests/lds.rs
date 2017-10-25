use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn lds_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LDS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 19], OperandSize::Word)
}

fn lds_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LDS, operand1: Some(Direct(DI)), operand2: Some(Indirect(EAX, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 197, 56], OperandSize::Dword)
}

fn lds_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LDS, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 60, 78], OperandSize::Dword)
}

