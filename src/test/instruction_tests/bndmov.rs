use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn bndmov_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND3)), operand2: Some(Direct(BND2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 218], OperandSize::Dword)
}

fn bndmov_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND0)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 2120562628, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 4, 245, 196, 55, 101, 126], OperandSize::Dword)
}

fn bndmov_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND3)), operand2: Some(Direct(BND0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 216], OperandSize::Qword)
}

fn bndmov_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND0)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 123513286, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 4, 141, 198, 169, 92, 7], OperandSize::Qword)
}

fn bndmov_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND1)), operand2: Some(Direct(BND2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 202], OperandSize::Dword)
}

fn bndmov_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 1346892772, Some(OperandSize::Qword), None)), operand2: Some(Direct(BND1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 27, 140, 218, 228, 243, 71, 80], OperandSize::Dword)
}

fn bndmov_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND2)), operand2: Some(Direct(BND1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 209], OperandSize::Qword)
}

fn bndmov_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(IndirectDisplaced(RSI, 939352563, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(BND3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 27, 158, 243, 97, 253, 55], OperandSize::Qword)
}

