use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn paddb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 196], OperandSize::Dword)
}

fn paddb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1792643561, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 140, 114, 233, 145, 217, 106], OperandSize::Dword)
}

fn paddb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 232], OperandSize::Qword)
}

fn paddb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(RDI, 1599618775, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 252, 191, 215, 62, 88, 95], OperandSize::Qword)
}

fn paddb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 251], OperandSize::Dword)
}

fn paddb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EBX, 563351987, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 131, 179, 17, 148, 33], OperandSize::Dword)
}

fn paddb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 255], OperandSize::Qword)
}

fn paddb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RSI, 770966491, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 252, 174, 219, 3, 244, 45], OperandSize::Qword)
}

