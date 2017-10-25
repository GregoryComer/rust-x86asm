use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn lfs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(BP, 32, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 180, 126, 32], OperandSize::Word)
}

fn lfs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(SI)), operand2: Some(Indirect(ESI, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 180, 54], OperandSize::Dword)
}

fn lfs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(RCX, 124603564, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 180, 177, 172, 76, 109, 7], OperandSize::Qword)
}

fn lfs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 960743693, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 180, 148, 83, 13, 201, 67, 57], OperandSize::Dword)
}

fn lfs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(EDX)), operand2: Some(Indirect(RCX, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 180, 17], OperandSize::Qword)
}

fn lfs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(RSP)), operand2: Some(IndirectDisplaced(RBX, 1843367487, Some(OperandSize::Far64), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 180, 163, 63, 142, 223, 109], OperandSize::Qword)
}

