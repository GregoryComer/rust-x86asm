use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 249, 53], OperandSize::Word)
}

#[test]
fn shld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 23, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 107, 23, 81], OperandSize::Word)
}

#[test]
fn shld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 235, 118], OperandSize::Dword)
}

#[test]
fn shld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 20, 176, 49], OperandSize::Dword)
}

#[test]
fn shld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 205, 115], OperandSize::Qword)
}

#[test]
fn shld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(RDX, 2042607570, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 170, 210, 183, 191, 121, 95], OperandSize::Qword)
}

#[test]
fn shld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 203, 25], OperandSize::Word)
}

#[test]
fn shld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(BX, 225, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 151, 225, 0, 115], OperandSize::Word)
}

#[test]
fn shld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EDI)), operand2: Some(Direct(ECX)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 207, 54], OperandSize::Dword)
}

#[test]
fn shld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(EAX, 299914685, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 152, 189, 85, 224, 17, 39], OperandSize::Dword)
}

#[test]
fn shld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 206, 67], OperandSize::Qword)
}

#[test]
fn shld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 12, 201, 76], OperandSize::Qword)
}

#[test]
fn shld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSI)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 164, 243, 18], OperandSize::Qword)
}

#[test]
fn shld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1759335901, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 164, 52, 213, 221, 85, 221, 104, 25], OperandSize::Qword)
}

#[test]
fn shld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 250], OperandSize::Word)
}

#[test]
fn shld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 29], OperandSize::Word)
}

#[test]
fn shld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 223], OperandSize::Dword)
}

#[test]
fn shld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 1920455266, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 60, 197, 98, 210, 119, 114], OperandSize::Dword)
}

#[test]
fn shld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 249], OperandSize::Qword)
}

#[test]
fn shld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 43], OperandSize::Qword)
}

#[test]
fn shld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 253], OperandSize::Word)
}

#[test]
fn shld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(SI, 167, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 140, 167, 0], OperandSize::Word)
}

#[test]
fn shld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 227], OperandSize::Dword)
}

#[test]
fn shld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 763862891, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 148, 87, 107, 159, 135, 45], OperandSize::Dword)
}

#[test]
fn shld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 213], OperandSize::Qword)
}

#[test]
fn shld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 792003927, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 60, 221, 87, 5, 53, 47], OperandSize::Qword)
}

#[test]
fn shld_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 165, 214], OperandSize::Qword)
}

#[test]
fn shld_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(RCX, 1750215584, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 165, 161, 160, 43, 82, 104], OperandSize::Qword)
}

